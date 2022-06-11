defmodule SerdeExamples.Native do
  use Rustler, otp_app: :serde_examples, crate: :serde_examples, mode: :release
  def vec_deserialize(type, size), do: error()
  def hashmap_deserialize(type, hashmap_size, size), do: error()
  def example_snowflake_arrow(), do: error()
  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
