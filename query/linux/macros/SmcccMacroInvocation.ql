import cpp

from MacroInvocation mi, Function f
where
  mi.getMacro().getName() = "arm_smccc_1_1_invoke" and
  mi.getEnclosingFunction() = f
select
  mi,
  f.getName()                as enclosing_function,
  mi.getLocation().getFile(),
  mi.getLocation().getStartLine()
