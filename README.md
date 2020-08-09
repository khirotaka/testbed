# Testbed
Testbet for building a Python module in Rust.


## Problem Establishment

```
testbed
├── __init__.py
└── engine
      ├── __init__.py
      └── modules.py
```

既に上のようなPythonで書かれたモジュールがあったとする。


## 1st step

```shell script
$ pip install maturin
$ cargo init --lib
```

を実行する。これにより、`src`, `Cargo.toml` が生成される。
次に、`pyproject.toml`を用意し以下を書き込む。

```toml
[build-system]
requires = ["maturin"]
build-backend = "maturin"
```

また、`Cargo.toml`に以下を加える。
```toml
# ~~~~~~~~~

[lib]
name = "testbed"
crate-type = ["cdylib"]

[dependencies.pyo3]
version = "0.11.1"
features = ["extension-module"]

# ~~~~~~~~~
```

これで、[PyO3/maturin](https://github.com/PyO3/maturin)を使った環境の準備が整う。  

`name=...` の所はPythonのモジュールと同じ物にする必要がある。

## 2nd step

`src/lib.rs` に処理を書き込む。

```rust
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}


#[pymodule]
fn testbed(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;

    Ok(())
}
```

次に、Python側のコードを準備する。`testbed/_rust/__init__.py`を作成し、次のように書き込む。

```python
from ..testbed import sum_as_string
```