# Horiz-RTOS (Horiz-Realtime-Operating-System)

基礎から設計された、生産設備、航空宇宙、およびエンターテインメント（パチンコ・パチスロ）分野向けの純粋な高信頼リアルタイムOS。
TRON (ITRON/μT-Kernel) 仕様をベースに設計。
外部依存を一切排除した「Zero-Dependency」設計により、完全な所有権と透明性を実現。

> [!WARNING]
> **現在のステータス: 開発初期プロトタイプ**
> 本プロジェクトは現在開発の初期段階にあります。基盤設計とスケルトン実装が中心であり、現時点ではビルドエラーが含まれる可能性があるほか、実機での動作は保証されません。

## 対応アーキテクチャ

Horiz-RTOS は、産業界のレガシーから最先端の航空宇宙用プロセッサまで、幅広いハードウェアをカバーしています。

### 対応アーキテクチャ一覧

| カテゴリ | アーキテクチャ | 主な用途・プロセッサ | Status |
| :--- | :--- | :--- | :--- |
| **モダン・汎用** | **ARM (Cortex-M / AArch64)** | STM32, Apple Silicon, 各種 MCU | Foundation |
| | **RISC-V (RV32 / RV64)** | ESP32-C3, SiFive, HPSC (NASA) | Foundation |
| | **x86_64** | 産業用 PC, Edge Server, シミュレータ | Foundation |
| | **LoongArch (64bit)** | 近代的プロセッサ, Loongson | Foundation |
| | **MIPS (32bit)** | ネットワーク機器, 産業用コントローラ | Foundation |
| **産業・FA** | **SuperH (SH-2/4)** | PLC, モーター制御, 産業用ロボット | Foundation |
| | **Renesas RX** | 現代的 FA 機器, スマートビルディング | Foundation |
| | **Renesas H8** | 家電制御, 車載周辺, H8/300H系 | Foundation |
| **パチンコ・遊技機** | **V850 / RH850** | 遊技機主制御ボード (デファクト), 車載 | Foundation |
| | **RL78** | 遊技機周辺制御, 低電力センサ | Foundation |
| **医療・ヘルスケア** | **MSP430 (16bit)** | ペースメーカー, 血糖値トランスミッタ | Foundation |
| | **AVR (8bit/32bit)** | 携帯型各種モニター, ローエンド医療機器 | Foundation |
| **レガシー・教育** | **Zilog Z80 / MOS 6502** | 歴史的組み込み機器, 教育用 | Foundation |
| | **Motorola 68000** | X68000, 初期産業用制御 | Foundation |
| **航空宇宙** | **PowerPC (32bit)** | RAD750, 衛星搭載コンピュータ | Foundation |
| | **SPARC (V8)** | LEON シリーズ (ESA/JAXA採用) | Foundation |

## 主要な信頼性・リアルタイム機能

- **TRON/ITRON 仕様準拠**: 産業・宇宙分野で実績のある API 体系を採用し、既存資産との親和性を確保。
- **Rust によるメモリ安全性**: `no_std` 環境下での実装により、メモリ破壊や競合状態を言語レベルで排除。
- **時間的・空間的隔離**: ARINC 653 相当のパーティショニング設計を視野に入れ、タスク間の干渉を物理的・論理的に防止。
- **決定論的スケジューリング**: 予測可能なコンテキストスイッチと最速の割り込み応答性能。

## ディレクトリ構造

- **src/api/**: TRON/ITRON 互換システムコール定義。
- **src/kernel/**: スケジューラ、同期機構（セマフォ等）、TCB管理。
- **src/arch/**: CPU アーキテクチャ依存部（コンテキスト保存・復元、ブート骨組み）。
- **src/lib.rs**: カーネルのエントリーポイント。
