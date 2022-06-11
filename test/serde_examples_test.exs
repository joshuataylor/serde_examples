defmodule SerdeExamplesTest do
  use ExUnit.Case
  doctest SerdeExamples

  test "greets the world" do
    assert SerdeExamples.hello() == :world
  end
end
