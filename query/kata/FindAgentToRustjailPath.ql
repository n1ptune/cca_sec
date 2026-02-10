import rust
import predicates.Calls
import predicates.RustjailFilter
import predicates.PathUtils

/* 4. 取第一条链（可改成 select many） */
from Function entry, Function target
where
  isAgentServiceImpl(entry) and
  isFromRustjail(target) and
  calls+(entry, target)
select
  entry.getName(),
  target.getName(),
  callStack(entry, target)
