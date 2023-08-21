# bats file_tags=tag:block
skip
@test "block/empty.lox" {
  run target/debug/lox test/cases/block/empty.lox

  [ "${lines[0]}" = "ok" ]
}
