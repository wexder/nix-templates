{
  outputs = _: {
    templates =
      {
        go = {
          description = "Development shell for go";
          path = ./templates/go;
        };
      };
  };
}
