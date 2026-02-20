import os
import re

def fix_no_mangle():
    arch_dir = r"d:\Dev\Horiz-Realtime-Operating-System\src\arch"
    files = [f for f in os.listdir(arch_dir) if f.endswith("_start.rs") or f == "riscv.rs"]
    for file in files:
        path = os.path.join(arch_dir, file)
        with open(path, "r", encoding="utf-8") as f:
            content = f.read()
        new_content = content.replace("#[no_mangle]", "#[unsafe(no_mangle)]")
        with open(path, "w", encoding="utf-8") as f:
            f.write(new_content)

def fix_context_rs():
    path = r"d:\Dev\Horiz-Realtime-Operating-System\src\arch\context.rs"
    with open(path, "r", encoding="utf-8") as f:
        content = f.read()
    
    # x86_64とarmの定義において、他のアーキテクチャが指定されている場合は除外する
    all_features = ["arch-m68k", "arch-z80", "arch-h8", "arch-arm", "arch-x86_64", 
                    "arch-powerpc", "arch-sparc", "arch-v850", "arch-rl78", 
                    "arch-sh", "arch-rx", "arch-msp430", "arch-avr", "arch-loongarch64", "arch-mips", "arch-m6502", "arch-arm64", "arch-riscv64"]
    
    # target_archが不正なもの（h8, z80, sh, rx, etc.）を削除し、featureのみの判定にする
    invalid_targets = ["m68k", "z80", "h8", "v850", "rl78", "sh", "rx", "m6502", "msp430", "avr"]
    for t in invalid_targets:
        # replace `target_arch = "z80", feature = "arch-z80"` => `feature = "arch-z80"`
        content = re.sub(rf'target_arch\s*=\s*"{t}",\s*feature\s*=\s*"arch-{t}"', rf'feature = "arch-{t}"', content)
        content = re.sub(rf'target_arch\s*=\s*"{t}"\s*,\s*feature\s*=\s*"arch-{t}"', rf'feature = "arch-{t}"', content)
        content = re.sub(rf'target_arch="{t}",feature="arch-{t}"', rf'feature="arch-{t}"', content)

    # Replace x86_64
    x86_features = [f for f in all_features if f != "arch-x86_64"]
    not_any_features = 'not(any(' + ', '.join(f'feature="{f}"' for f in x86_features) + '))'
    
    content = content.replace(
        '#[cfg(any(target_arch = "x86_64", feature = "arch-x86_64"))]',
        f'#[cfg(any(feature = "arch-x86_64", all(target_arch = "x86_64", {not_any_features})))]'
    )
    
    # Replace arm
    arm_features = [f for f in all_features if f != "arch-arm"]
    not_any_features_arm = 'not(any(' + ', '.join(f'feature="{f}"' for f in arm_features) + '))'
    
    content = content.replace(
        '#[cfg(any(target_arch = "arm", feature = "arch-arm"))]',
        f'#[cfg(any(feature = "arch-arm", all(target_arch = "arm", {not_any_features_arm})))]'
    )

    with open(path, "w", encoding="utf-8") as f:
        f.write(content)

if __name__ == "__main__":
    fix_no_mangle()
    fix_context_rs()
    print("Fixed source files.")
