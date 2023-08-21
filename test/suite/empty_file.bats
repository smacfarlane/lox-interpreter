# bats file_tags=tag:basic
@test "empty_file.lox" {
  run target/debug/lox test/cases/empty_file.lox

}
