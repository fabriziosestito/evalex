defmodule EvalExTest do
  use ExUnit.Case
  use ExUnitProperties

  describe "eval/2" do
    test "should evaluate an expression without a context" do
      assert {:ok, 3} == EvalEx.eval("1 + 2")
    end

    test "should evaluate an expression with a context" do
      assert {:ok, 3} == EvalEx.eval("a + b", %{"a" => 1, "b" => 2})
    end

    test "should assing a variable" do
      assert {:ok, 7} == EvalEx.eval("c = 4; a + b + c", %{"a" => 1, "b" => 2})
    end

    test "should return an error if a variable does not exist" do
      assert {:error,
              {:variable_identifier_not_found,
               "Variable identifier is not bound to anything by context: \"b\"."}} ==
               EvalEx.eval("a + b", %{"a" => 1})
    end
  end

  describe "type conversion" do
    property "should convert integer() to Integer and Integer to integer()" do
      check all int <- integer() do
        assert {:ok, result} = EvalEx.eval("a", %{"a" => int})
        assert int == result
        assert is_integer(result)
      end
    end

    property "should convert float() to Float and Float to float()" do
      check all float <- float() do
        assert {:ok, result} = EvalEx.eval("a", %{"a" => float})
        assert float == result
        assert is_float(result)
      end
    end

    property "should convert boolean() to Bool and Bool to boolean()" do
      check all bool <- boolean() do
        assert {:ok, result} = EvalEx.eval("a", %{"a" => bool})
        assert bool == result
        assert is_boolean(result)
      end
    end

    property "should convert tuple() to Tuple and Tuple to list()" do
      check all tuple <- tuple({integer(), string(:ascii)}) do
        assert {:ok, result} = EvalEx.eval("a", %{"a" => tuple})
        assert Tuple.to_list(tuple) == result
        assert is_list(result)
      end
    end

    property "should convert list() to Tuple and Tuple to list()" do
      check all list <- list_of(string(:ascii)) do
        assert {:ok, result} = EvalEx.eval("a", %{"a" => list})
        assert list == result
        assert is_list(list)
      end
    end

    property "should convert String.t() to String and String to String.t()" do
      check all str1 <- string(:ascii),
                str2 <- string(:printable) do
        assert {:ok, result} = EvalEx.eval("a + b", %{"a" => str1, "b" => str2})
        assert str1 <> str2 == result
        assert is_binary(result)
      end
    end
  end
end
