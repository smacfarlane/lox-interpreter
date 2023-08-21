# bats file_tags=tag:variable
@test "variable/unreached_undefined.lox" {
  run target/debug/lox test/cases/variable/unreached_undefined.lox

  [ "${lines[0]}" = "ok" ]
}
