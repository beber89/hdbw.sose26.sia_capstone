FROM golang:1.21

WORKDIR /workspace

COPY go.mod ./

COPY . .

EXPOSE 8080

CMD ["go", "run", "."]
