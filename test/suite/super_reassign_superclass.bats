# bats file_tags=tag:super
skip
@test "super/reassign_superclass.lox" {
  run target/debug/lox test/cases/super/reassign_superclass.lox

  [ "${lines[0]}" = "Base.method()" ]
  [ "${lines[1]}" = "Base.method()" ]
}
