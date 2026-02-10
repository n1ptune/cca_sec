import cpp

from MacroInvocation mi, Expr smcId, Function f
where
  mi.getMacro().getName() = "arm_smccc_1_1_invoke" and
  mi.getEnclosingFunction() = f and

  // 宏展开后的第一个表达式 = SMC ID
  smcId = mi.getExpansion().getAChildExpr()
select
  f.getName()        as enclosing_function,
  smcId.toString()  as smc_id,
  mi.getLocation().getFile(),
  mi.getLocation().getStartLine()
