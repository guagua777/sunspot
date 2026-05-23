module sunspot/go

go 1.24.2

// 两个 require 块是 Go 的惯例，用来区分两类依赖：

// 第一个 require（第5-12行）：你直接使用的依赖，即代码中 import 的包。

// 第二个 require（第14-34行）：标记为 // indirect 的间接依赖，即你的直接依赖所需要的包，你的代码本身并不直接 import 它们。

// 这个分隔是 go mod tidy 自动维护的，方便一眼看出哪些是你真正关心的依赖，哪些只是传递依赖。


require (
	github.com/consensys/gnark v0.14.0
	github.com/consensys/gnark-crypto v0.19.0
	github.com/google/btree v1.0.0
	github.com/rs/zerolog v1.34.0
	github.com/spf13/cobra v1.10.1
	github.com/tidwall/btree v1.7.0
)

require (
	github.com/bits-and-blooms/bitset v1.24.0 // indirect
	github.com/blang/semver/v4 v4.0.0 // indirect
	github.com/davecgh/go-spew v1.1.1 // indirect
	github.com/fxamacker/cbor/v2 v2.9.0 // indirect
	github.com/google/pprof v0.0.0-20250820193118-f64d9cf942d6 // indirect
	github.com/inconshreveable/mousetrap v1.1.0 // indirect
	github.com/ingonyama-zk/icicle-gnark/v3 v3.2.2 // indirect
	github.com/mattn/go-colorable v0.1.14 // indirect
	github.com/mattn/go-isatty v0.0.20 // indirect
	github.com/pmezard/go-difflib v1.0.0 // indirect
	github.com/ronanh/intcomp v1.1.1 // indirect
	github.com/spf13/pflag v1.0.9 // indirect
	github.com/stretchr/testify v1.10.0 // indirect
	github.com/x448/float16 v0.8.4 // indirect
	golang.org/x/crypto v0.41.0 // indirect
	golang.org/x/exp v0.0.0-20250819193227-8b4c13bb791b // indirect
	golang.org/x/sync v0.16.0 // indirect
	golang.org/x/sys v0.35.0 // indirect
	gopkg.in/yaml.v3 v3.0.1 // indirect
)
