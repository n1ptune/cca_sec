import rust

/* 2. 限定 rustjail crate */
predicate isFromRustjail(Function f) {
  exists(Impl impl |
    impl.getSelfTy().toString().regexpMatch(".*LinuxContainer.*") and
    f = impl.getAssocItemList().(AssocItemList).getAnAssocItem().(Function)
  )
}

predicate isAgentServiceImpl(Function f) {
  exists(Impl impl |
    impl.getSelfTy().toString().matches("AgentService%") and
    f = impl.getAssocItemList().(AssocItemList).getAnAssocItem().(Function)
  )
}
