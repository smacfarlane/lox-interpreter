# bats file_tags=tag:super
skip
@test "super/call_other_method.lox" {
  run target/debug/lox test/cases/super/call_other_method.lox

  [ "${lines[0]}" = "Derived.bar()" ]
  [ "${lines[1]}" = "Base.foo()" ]
}
