# bats file_tags=tag:super
skip
@test "super/indirectly_inherited.lox" {
  run target/debug/lox test/cases/super/indirectly_inherited.lox

  [ "${lines[0]}" = "C.foo()" ]
  [ "${lines[1]}" = "A.foo()" ]
}
