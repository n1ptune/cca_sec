import cpp

from Function caller,
     Function smcWrapper,
     FunctionCall call,
     MacroInvocation mi
where
  // wrapper 内部确实调用了宏
  mi.getMacro().getName() = "arm_smccc_1_1_invoke" and
  mi.getEnclosingFunction() = smcWrapper and

  // caller → smcWrapper
  call.getTarget() = smcWrapper and
  call.getEnclosingCallable() = caller
select
  caller.getName()     as caller,
  smcWrapper.getName() as smc_wrapper,
  call.getLocation().getFile(),
  call.getLocation().getStartLine()
