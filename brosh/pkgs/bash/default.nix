{
  bash,
}:
bash.overrideAttrs (oldAttrs: {
  outputs = (oldAttrs.outputs or [ "out" ]) ++ [ "tests" ];

  postInstall = (oldAttrs.postInstall or "") + ''
    mkdir -p $tests
    cp -r ${oldAttrs.src}/tests $tests/
  '';
})
