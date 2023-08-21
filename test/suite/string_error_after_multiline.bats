# bats file_tags=tag:string
@test "string/error_after_multiline.lox" {
  run target/debug/lox test/cases/string/error_after_multiline.lox

}
