import cpp

from Function caller,
     Function smcWrapper,
     FunctionCall call,
     MacroInvocation mi,
     Expr smcId
where
  // 1. smcWrapper 内部调用了 arm_smccc_1_1_invoke 宏
  mi.getMacro().getName() = "arm_smccc_1_1_invoke" and
  mi.getEnclosingFunction() = smcWrapper and

  // 2. 取 SMC ID（宏的第一个参数，index 0）
  smcId = mi.getExpr().getChild(0) and

  // 3. caller 调用了 smcWrapper
  call.getTarget() = smcWrapper and
  call.getEnclosingFunction() = caller

select
  caller,
  caller.getName(),
  smcWrapper.getName(),
  smcId.toString(),
  call.getLocation()