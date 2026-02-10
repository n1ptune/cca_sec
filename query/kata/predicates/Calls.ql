import rust

/* 1. 直接调用边（静态函数 + 方法） */
predicate calls(Function caller, Function callee) {
  exists(CallExpr call |
    call.getEnclosingCallable() = caller and
    call.getStaticTarget() = callee
  )
  or
  exists(MethodCallExpr mce |
    mce.getEnclosingCallable() = caller and
    mce.getIdentifier().toString() = callee.getName().toString()
  )
}
