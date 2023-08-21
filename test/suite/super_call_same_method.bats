# bats file_tags=tag:super
skip
@test "super/call_same_method.lox" {
  run target/debug/lox test/cases/super/call_same_method.lox

  [ "${lines[0]}" = "Derived.foo()" ]
  [ "${lines[1]}" = "Base.foo()" ]
}
