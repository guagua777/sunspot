package cmd

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"sunspot/go/acir"

	"github.com/spf13/cobra"
)

var compileCmd = &cobra.Command{
	Use:   "compile [acir_file]",
	Short: "Compile an ACIR file into a CCS file",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		acirPath := args[0]

		// 必须为json文件
		if filepath.Ext(acirPath) != ".json" {
			return fmt.Errorf("invalid input file: %s (must end with .json)", acirPath)
		}
		fmt.Printf("Loading ACIR file: %s\n", acirPath)

		// 加载ACIR文件 命令行中输入的参数
		acir, err := acir.LoadACIR[T, E](acirPath)

		if err != nil {
			return fmt.Errorf("failed to load ACIR: %v", err)
		}

		// 编译ACIR文件
		ccs, err := acir.Compile()
		if err != nil {
			return fmt.Errorf("failed to compile ACIR: %v", err)
		}

		fmt.Println("Compilation successful.")

		// 生成输出文件路径
		// 去掉json后缀
		base := strings.TrimSuffix(acirPath, ".json")
		outPath := base + ".ccs"

		// Open output file for writing
		outFile, err := os.Create(outPath)
		if err != nil {
			return fmt.Errorf("failed to create CCS file: %w", err)
		}
		// 关闭文件
		defer outFile.Close()

		// Write CCS to file using its WriteTo() method
		if _, err := ccs.WriteTo(outFile); err != nil {
			return fmt.Errorf("failed to write CCS file: %w", err)
		}

		fmt.Printf("💾 CCS written to %s\n", outPath)

		return nil
	},
}
