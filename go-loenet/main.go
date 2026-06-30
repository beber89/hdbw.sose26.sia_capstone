package main

import (
    "encoding/json"
    "fmt"
    "net"
    "net/http"
    "sync"
    "golang.org/x/net/websocket"
)

// Message represents the structure of the JSON message
type Message struct {
    Header struct {
        DestinationIP   string `json:"destination_ip"`
        DestinationPort string `json:"destination_port"`
    } `json:"header"`
    Payload string `json:"payload"`
}

// Client represents a connected client
type Client struct {
    Conn *websocket.Conn
    IP   string
    Port string
}

var (
    clients     = make(map[string]*Client)
    clientsLock sync.Mutex
)

func main() {
    http.Handle("/ws", websocket.Handler(wsHandler))
    fmt.Println("Message Center started at :8080")
    http.ListenAndServe(":8080", nil)
}

func wsHandler(ws *websocket.Conn) {
    // Extract client IP and port
    addr := ws.Request().RemoteAddr
    host, port, err := net.SplitHostPort(addr)
    if err != nil {
        fmt.Println("Error parsing client address: ", err)
        ws.Close()
        return
    }
    
    // Register client
    clientKey := fmt.Sprintf("%s:%s", host, port)
    clientsLock.Lock()
    clients[clientKey] = &Client{Conn: ws, IP: host, Port: port}
    clientsLock.Unlock()
    
    defer func() {
        clientsLock.Lock()
        delete(clients, clientKey)
        clientsLock.Unlock()
        ws.Close()
    }()
    
    fmt.Printf("New WebSocket connection from %s\n", clientKey)
    
    for {
        var msg Message
        err := websocket.JSON.Receive(ws, &msg)
        if err != nil {
            fmt.Println("Error receiving message: ", err)
            break
        }
        
        destinationKey := fmt.Sprintf("%s:%s", msg.Header.DestinationIP, msg.Header.DestinationPort)
        fmt.Printf("Routing message from %s to %s: %s\n", clientKey, destinationKey, msg.Payload)
        
        clientsLock.Lock()
        destinationClient, exists := clients[destinationKey]
        clientsLock.Unlock()
        
        if exists {
            err = websocket.JSON.Send(destinationClient.Conn, msg)
            if err != nil {
                fmt.Printf("Error sending message to %s: %v\n", destinationKey, err)
            }
        } else {
            fmt.Printf("Destination client %s not found\n", destinationKey)
        }
    }
}