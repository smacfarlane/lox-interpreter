# bats file_tags=tag:super
skip
@test "super/constructor.lox" {
  run target/debug/lox test/cases/super/constructor.lox

  [ "${lines[0]}" = "Derived.init()" ]
  [ "${lines[1]}" = "Base.init(a, b)" ]
}
