{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    docker
    docker-compose
    kubectl
  ];

  shellHook = ''
    echo "Docker environment ready"
    echo "Docker version: $(docker version --format '{{.Server.Version}}' 2>/dev/null || echo 'not connected')"
    echo ""
    echo "To start containers:"
    echo "  docker-compose up --build"
    echo ""
  '';

  DOCKER_HOST = "unix:///var/run/docker.sock";
}