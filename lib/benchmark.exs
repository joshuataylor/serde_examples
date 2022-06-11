Benchee.run(
  %{
    "vec with 30,000 str items" => fn -> SerdeExamples.Native.vec_deserialize("str", 50000) end,
    "vec with 30,000 integer items" => fn ->
      SerdeExamples.Native.vec_deserialize("integer", 50000)
    end,
    "vec with 30,000 string items" => fn ->
      SerdeExamples.Native.vec_deserialize("string", 50000)
    end,
    "vec with 100 str items" => fn -> SerdeExamples.Native.vec_deserialize("str", 100) end,
    "vec with 100 integer items" => fn ->
      SerdeExamples.Native.vec_deserialize("integer", 100)
    end,
    "vec with 100 string items" => fn ->
      SerdeExamples.Native.vec_deserialize("string", 100)
    end,
    "hashmap of size 4 with 30,000 str items" => fn ->
      SerdeExamples.Native.hashmap_deserialize("str", 4, 50000)
    end,
    "hashmap of size 4 vec with 30,000 integer items" => fn ->
      SerdeExamples.Native.hashmap_deserialize("integer", 4, 50000)
    end,
    "hashmap of size 4 vec with 30,000 string items" => fn ->
      SerdeExamples.Native.hashmap_deserialize("string", 4, 50000)
    end,
    "hashmap of size 100 with 30,000 str items" => fn ->
      SerdeExamples.Native.hashmap_deserialize("str", 100, 50000)
    end,
    "hashmap of size 100 with 30,000 integer items" => fn ->
      SerdeExamples.Native.hashmap_deserialize("integer", 100, 50000)
    end,
    "hashmap of size 100 with 30,000 string items" => fn ->
      SerdeExamples.Native.hashmap_deserialize("string", 100, 50000)
    end,
    "Example of snowflake_arrow, where we we get back 30,000 rows with 10 " => fn ->
      SerdeExamples.Native.example_snowflake_arrow()
    end

  },
  time: 5
)
