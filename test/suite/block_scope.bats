# bats file_tags=tag:block
skip
@test "block/scope.lox" {
  run target/debug/lox test/cases/block/scope.lox

  [ "${lines[0]}" = "inner" ]
  [ "${lines[1]}" = "outer" ]
}
