import cpp

from MacroInvocation outerMi,
     MacroInvocation innerMi,
     MacroInvocation smcIdMi,
     Function f,
     int col
where
  // 1. 最外层 arm_smccc_1_1_invoke
  outerMi.getMacro().getName() = "arm_smccc_1_1_invoke" and
  outerMi.getEnclosingFunction() = f and

  // 2. 中间层 arm_smccc_1_1_smc / arm_smccc_1_1_hvc
  innerMi.getParentInvocation() = outerMi and
  innerMi.getMacro().getName().matches("arm_smccc_1_1_%") and

  // 3. SMC ID 宏是 innerMi 的直接子宏，列号最小即第一个参数
  smcIdMi.getParentInvocation() = innerMi and
  col = smcIdMi.getLocation().getStartColumn() and
  not exists(MacroInvocation earlier |
    earlier.getParentInvocation() = innerMi and
    earlier.getLocation().getStartColumn() < col
  )

select
  f.getName(),
  smcIdMi.getMacro().getName(),
  outerMi.getLocation().getFile(),
  outerMi.getLocation().getStartLine()