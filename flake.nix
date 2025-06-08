{
  outputs = _: {
    templates =
      {
        go = {
          description = "Development shell for go";
          path = ./templates/go;
        };
        rust = {
          description = "Development shell for rust";
          path = ./templates/rust;
        };
      };
  };
}
