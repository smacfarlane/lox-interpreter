# bats file_tags=tag:comments
@test "comments/unicode.lox" {
  run target/debug/lox test/cases/comments/unicode.lox

  [ "${lines[0]}" = "ok" ]
}
