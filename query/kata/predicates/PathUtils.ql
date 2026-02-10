import rust
import predicates.Calls

/* 3. 递归拼路径：入口 → target */
string buildPath(Function entry, Function to, int depth) {
  (depth = 1 and
   calls(entry, to) and
   result = entry.getName() + " → " + to.getName())
  or
  (depth > 0 and
   exists(Function mid |
     calls(entry, mid) and
     calls+(mid, to) |
     result = entry.getName() + " → " + buildPath(mid, to, depth-1)
   )
  )
}

string callStack(Function entry, Function target) {
  calls+(entry, target) and
  result = concat(
    Function f |
      calls*(entry, f) and calls+(f, target)
    |
      f.getName().toString() + "#" +
      f.getLocation().getFile() + "@" +
      f.getLocation().getStartLine(),
    " → "
    order by f.getLocation().getStartLine()
  )
}
