# bats file_tags=tag:function
skip
@test "function/body_must_be_block.lox" {
  run target/debug/lox test/cases/function/body_must_be_block.lox

}
