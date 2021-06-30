# Second State ｜ 用 Docker 工具管理 Rust 函数

作者：夏歌

---

开发者可以通过 DockerHub 和 CRI-O 等 Docker 工具在 WasmEdge 中部署、管理和运行轻量级 WebAssembly 应用程序。

> [WasmEdge](https://github.com/WasmEdge/WasmEdge)  是由 [CNCF (Cloud Native Computing Foundation) 托管的](https://www.cncf.io/sandbox-projects/) WebAssembly 运行时，是边缘计算应用程序的执行沙箱。

虽然 WebAssembly 最初是作为浏览器应用程序的运行时而发明的，但其轻量级和高性能的沙箱设计使其成为通用应用程序容器的一个极具吸引力的选择。

> 如果在 2008 年已经有了 WASM + WASI，那么我们压根无需创始 Docker 这个项目了。 — Docker 联合创始人 Solomon Hykes

与 Docker 相比， [WebAssembly 在启动时快一百倍](https://www.infoq.com/articles/arm-vs-x86-cloud-performance/), 占用更小的内存和磁盘空间，并且具有更优定义的安全沙箱。然而，缺点是 WebAssembly 需要自己的语言 SDK 和编译器工具链，使其作为开发者环境比 Docker 更受限制。WebAssembly 越来越多地用于边缘计算场景，通常这些场景中，部署 Docker 比较困难，或是应用程序的性能至关重要。

Docker 的一大优势是其丰富的工具生态系统。我们希望为 WasmEdge 开发者带来类似 Docker 的工具。为了实现这一点，我们为 CRI-O 创建了一个名为 [runw](https://github.com/second-state/runw) 的替代 runner 来加载并运行 WebAssembly 字节码程序，如同他们是 Docker 镜像文件一样。


## 在 CRI-O 中安装 WebAssembly runner

为了在 CRI-O 中支持 WebAssembly，您只需下载 `runw` 二进制码发布并将其安装到您的 CRI-O 中。

> 因为 `runw` 二进制码已经包括了 WasmEdge，无需单独安装 WasmEdge 或任何其它 WebAssembly VM。

首先，确保你使用的是安装了 LLVM-10 的 Ubuntu 20.04。如果你使用的是不同的平台，请参阅[如何为你的操作系统创建 `runw` 项目文档。](https://github.com/second-state/runw#build-from-source)


```
sudo apt install -y llvm-10-dev liblld-10-dev
```


确保你安装了 [cri-o](https://cri-o.io/)、 [crictl](https://github.com/kubernetes-sigs/cri-tools)、 [containernetworking-plugins](https://github.com/containernetworking/plugins) 和 [buildah](https://github.com/containers/buildah) 或者 [docker](https://github.com/docker/cli) 。

下一步，下载 `runw` binary build

```
wget https://github.com/second-state/runw/releases/download/0.1.0/runw
```

现在，你可以安装 `runw` 进 CRI-O 作为 WebAssembly 的备选方案。

```
# Get the wasm-pause utility
sudo crictl pull docker.io/beststeve/wasm-pause

# Install runw into cri-o
sudo cp -v runw /usr/lib/cri-o-runc/sbin/runw
sudo chmod +x /usr/lib/cri-o-runc/sbin/runw
sudo sed -i -e 's@default_runtime = "runc"@default_runtime = "runw"@' /etc/crio/crio.conf
sudo sed -i -e 's@pause_image = "k8s.gcr.io/pause:3.2"@pause_image = "docker.io/beststeve/wasm-pause"@' /etc/crio/crio.conf
sudo sed -i -e 's@pause_command = "/pause"@pause_command = "pause.wasm"@' /etc/crio/crio.conf
sudo tee -a /etc/crio/crio.conf.d/01-crio-runc.conf <<EOF
[crio.runtime.runtimes.runw]
runtime_path = "/usr/lib/cri-o-runc/sbin/runw"
runtime_type = "oci"
runtime_root = "/run/runw"
EOF
```


最后，重启 `cri-o` ，从而使新的 WebAssembly runner 开始生效。


```
sudo systemctl restart crio
```

## 用 Rust 构建 Wasm 应用程序

下面案例中的 Wasm 应用程序是 Rust 写的。为了让这些程序工作，确保你安装了 [Rust](https://www.rust-lang.org/tools/install) 和 [rustwasmc](https://www.secondstate.io/articles/rustwasmc/) 工具链。


> 你需要 Rust 编译器和 rustwasmc 来构建 Rust 源成为 wasm 字节码文件。如果你已经有一个 wasm字节码程序，且只是想要用 cri-o 跑一遍，你可以跳过这个部分。


应用程序源代码仅为一个 [`main.rs`](http://main.rs/) 函数。[在此处。](https://github.com/second-state/wasm-learning/tree/master/ssvm/wasi)该应用程序演示了如何使用标准 Rust API 从 WasmEdge 访问文件系统和其它操作系统资源。


```
fn main() {
  println!("Random number: {}", get_random_i32());
  println!("Random bytes: {:?}", get_random_bytes());
  println!("{}", echo("This is from a main function"));
  print_env();
  create_file("/tmp.txt", "This is in a file");
  println!("File content is {}", read_file("/tmp.txt"));
  del_file("/tmp.txt");
}

pub fn get_random_i32() -> i32 {
  let x: i32 = random();
  return x;
}

pub fn get_random_bytes() -> Vec<u8> {
  let mut rng = thread_rng();
  let mut arr = [0u8; 128];
  rng.fill(&mut arr[..]);
  return arr.to_vec();
}

pub fn echo(content: &str) -> String {
  println!("Printed from wasi: {}", content);
  return content.to_string();
}

pub fn print_env() {
  println!("The env vars are as follows.");
  for (key, value) in env::vars() {
    println!("{}: {}", key, value);
  }

  println!("The args are as follows.");
  for argument in env::args() {
    println!("{}", argument);
  }
}

pub fn create_file(path: &str, content: &str) {
  let mut output = File::create(path).unwrap();
  output.write_all(content.as_bytes()).unwrap();
}

pub fn read_file(path: &str) -> String {
  let mut f = File::open(path).unwrap();
  let mut s = String::new();
  match f.read_to_string(&mut s) {
    Ok(_) => s,
    Err(e) => e.to_string(),
  }
}

pub fn del_file(path: &str) {
  fs::remove_file(path).expect("Unable to delete");
}
```


你可以通过下面的命令行将应用程序构建到一个 wasm 字节码文件中。


```
rustwasmc build
```


 wasm 字节码文件 [在这里。](https://github.com/second-state/wasm-learning/blob/master/ssvm/wasi/wasi_example_main.wasm) 


## 为 Wasm app 构建并发布一个 Docker Hub 镜像

您现在可以将整个 wasm 字节码文件发布到 Docker hub 中，就好像这是一个 Docker 镜像一样。

首先，在  `pkg/` 目录中创建一个 Dockerfile，如下所示。

```
FROM scratch
ADD wasi_example_main.wasm .
CMD ["wasi_example_main.wasm"]
```


创建一个镜像并发布到 Docker hub。


```
sudo buildah bud -f Dockerfile -t wasm-wasi-example
sudo buildah push wasm-wasi-example docker://registry.example.com/repository:tag

# Example: the following command publishes the wasm image to the public Docker hub under user account "hydai"
sudo buildah push wasm-wasi-example docker://docker.io/hydai/wasm-wasi-example:latest
```


现在，你可以使用 Docker 工具（例如 `crictl`）将发布的 wasm 文件拉为镜像。 下面是我们发布的 wasm 文件镜像的示例。


```
sudo crictl pull docker.io/hydai/wasm-wasi-example
```



## 使用 CRI-O 启动 Wasm app

要启动并运行 wasm 文件，您需要为 CRI-O 创建两个配置文件。 创建一个 `container_wasi.json` 文件，如下所示。 它告诉 CRI-O 运行时应该从 Docker 存储库的哪里提取 wasm 文件映像。


```
{
  "metadata": {
    "name": "podsandbox1-wasm-wasi"
  },
  "image": {
    "image": "hydai/wasm-wasi-example:latest"
  },
  "args": [
    "wasi_example_main.wasm", "50000000"
  ],
  "working_dir": "/",
  "envs": [],
  "labels": {
    "tier": "backend"
  },
  "annotations": {
    "pod": "podsandbox1"
  },
  "log_path": "",
  "stdin": false,
  "stdin_once": false,
  "tty": false,
  "linux": {
    "resources": {
      "memory_limit_in_bytes": 209715200,
      "cpu_period": 10000,
      "cpu_quota": 20000,
      "cpu_shares": 512,
      "oom_score_adj": 30,
      "cpuset_cpus": "0",
      "cpuset_mems": "0"
    },
    "security_context": {
      "namespace_options": {
        "pid": 1
      },
      "readonly_rootfs": false,
      "capabilities": {
        "add_capabilities": [
          "sys_admin"
        ]
      }
    }
  }
}
```


接下来，创建一个 `sandbox_config.json` 文件，如下所示。 它定义了运行 wasm 应用程序的沙箱环境。


```
{
  "metadata": {
    "name": "podsandbox12",
    "uid": "redhat-test-crio",
    "namespace": "redhat.test.crio",
    "attempt": 1
  },
  "hostname": "crictl_host",
  "log_directory": "",
  "dns_config": {
    "searches": [
      "8.8.8.8"
    ]
  },
  "port_mappings": [],
  "resources": {
    "cpu": {
      "limits": 3,
      "requests": 2
    },
    "memory": {
      "limits": 50000000,
      "requests": 2000000
    }
  },
  "labels": {
    "group": "test"
  },
  "annotations": {
    "owner": "hmeng",
    "security.alpha.kubernetes.io/seccomp/pod": "unconfined"
  },
  "linux": {
    "cgroup_parent": "pod_123-456.slice",
    "security_context": {
      "namespace_options": {
        "network": 0,
        "pid": 1,
        "ipc": 0
      },
      "selinux_options": {
        "user": "system_u",
        "role": "system_r",
        "type": "svirt_lxc_net_t",
        "level": "s0:c4,c5"
      }
    }
  }
}
```


现在可以创建一个 CRI-O pod 如下：


```
# 创建 POD，输出将会和示例不同。
sudo crictl runp sandbox_config.json
7992e75df00cc1cf4bff8bff660718139e3ad973c7180baceb9c84d074b516a4

# 设置一个辅助变量供以后使用。
POD_ID=7992e75df00cc1cf4bff8bff660718139e3ad973c7180baceb9c84d074b516a4
```


自 pod ，您可以创建一个容器以隔离方式运行 wasm 字节码程序。


```
# 创建容器实例，输出将会和示例不同。
sudo crictl create $POD_ID container_wasi.json sandbox_config.json
1d056e4a8a168f0c76af122d42c98510670255b16242e81f8e8bce8bd3a4476f
```


最后，启动容器并查看 wasm 应用程序的输出。


```
# 列出容器，状态应该是 `Created`
sudo crictl ps -a

CONTAINER           IMAGE                           CREATED              STATE               NAME                     ATTEMPT             POD ID
1d056e4a8a168       hydai/wasm-wasi-example:latest   About a minute ago   Created             podsandbox1-wasm-wasi   0                   7992e75df00cc

# 启动容器
sudo crictl start 1d056e4a8a168f0c76af122d42c98510670255b16242e81f8e8bce8bd3a4476f
1d056e4a8a168f0c76af122d42c98510670255b16242e81f8e8bce8bd3a4476f

# 再次检查容器状态。#如果容器没有完成工作，你会看到运行状态。 #因为这个例子很小。此时您可能会看到 Exited。
sudo crictl ps -a
CONTAINER           IMAGE                           CREATED              STATE               NAME                     ATTEMPT             POD ID
1d056e4a8a168       hydai/wasm-wasi-example:latest   About a minute ago   Running             podsandbox1-wasm-wasi   0                   7992e75df00cc

# 当容器完成。你能看到状态变为 Exited。
sudo crictl ps -a
CONTAINER           IMAGE                           CREATED              STATE               NAME                     ATTEMPT             POD ID
1d056e4a8a168       hydai/wasm-wasi-example:latest   About a minute ago   Exited              podsandbox1-wasm-wasi   0                   7992e75df00cc

# 查看容器记录 
sudo crictl logs 1d056e4a8a168f0c76af122d42c98510670255b16242e81f8e8bce8bd3a4476f

Test 1: 打印随机数
Random number: 960251471

Test 2: 打印随机字节
Random bytes: [50, 222, 62, 128, 120, 26, 64, 42, 210, 137, 176, 90, 60, 24, 183, 56, 150, 35, 209, 211, 141, 146, 2, 61, 215, 167, 194, 1, 15, 44, 156, 27, 179, 23, 241, 138, 71, 32, 173, 159, 180, 21, 198, 197, 247, 80, 35, 75, 245, 31, 6, 246, 23, 54, 9, 192, 3, 103, 72, 186, 39, 182, 248, 80, 146, 70, 244, 28, 166, 197, 17, 42, 109, 245, 83, 35, 106, 130, 233, 143, 90, 78, 155, 29, 230, 34, 58, 49, 234, 230, 145, 119, 83, 44, 111, 57, 164, 82, 120, 183, 194, 201, 133, 106, 3, 73, 164, 155, 224, 218, 73, 31, 54, 28, 124, 2, 38, 253, 114, 222, 217, 202, 59, 138, 155, 71, 178, 113]

Test 3: 调用 echo 函数
Printed from wasi: This is from a main function
This is from a main function

Test 4: 打印环境变量
The env vars are as follows.
PATH: /usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
TERM: xterm
HOSTNAME: crictl_host
PATH: /usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
The args are as follows.
/var/lib/containers/storage/overlay/006e7cf16e82dc7052994232c436991f429109edea14a8437e74f601b5ee1e83/merged/wasi_example_main.wasm
50000000

Test 5: 创建文件 `/tmp.txt` 包含内容 `This is in a file`

Test 6: 从之前文件读取内容
File content is This is in a file

Test 7: 删除之前文件
```



## 下一步

在本文中，我们看到了如何使用类似 Docker 的 CRI-O 工具启动、运行和管理 WasmEdge 应用程序。

我们的下一步是使用 Kubernetes 来管理 WasmEdge 容器。 为此，我们需要在 Kubernetes 中安装一个 runner 二进制文件，以便它可以同时支持常规 Docker 镜像和 wasm 字节码镜像。


## Second State 介绍

Second State 专注 WebAssembly 生态，其开源项目 [WasmEdge](https://github.com/WasmEdge/WasmEdge) 已经成为 CNCF 沙箱项目。WasmEdge 是为边缘计算优化的 mission critical、实时、轻量级、高性能软件执行环境（也称 runtime 或者虚拟机）。


 






