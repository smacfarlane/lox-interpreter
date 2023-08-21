# bats file_tags=tag:comments
@test "comments/line_at_eof.lox" {
  run target/debug/lox test/cases/comments/line_at_eof.lox

  [ "${lines[0]}" = "ok" ]
}
