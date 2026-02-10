/**
 * @name 查找 RMI / Realm 相关函数
 * @kind problem
 * @id c/rmi-realm-functions
 */

import cpp

from Function f
where
  f.getName().regexpMatch(".*(rmi_|realm_|rsi_|smc_|hvc_).*")
select
  f,
  f.getLocation().getFile().getRelativePath() + ":" + f.getLocation().getStartLine(),
  "RMM侧处理调用"
