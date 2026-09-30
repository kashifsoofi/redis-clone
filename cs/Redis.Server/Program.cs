using System.Net;
using System.Net.Sockets;
using System.Text;

class Program
{
    static void Main(string[] args)
    {
        var ipAddress = "127.0.0.1";
        var port = 6379;
        var localEndPoint = new IPEndPoint(IPAddress.Parse(ipAddress), port);
        
        var socket = new Socket(AddressFamily.InterNetwork, SocketType.Stream, ProtocolType.Tcp);

        try
        {
            socket.Bind(localEndPoint);
            socket.Listen(10);

            Console.WriteLine($"Server is listening on port {port}");

            while (true)
            {
                var clientSocket = socket.Accept();
                
                var clientThread = new Thread(() => HandleClient(clientSocket));
                clientThread.Start();
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.ToString());
        }
    }

    private static void HandleClient(Socket clientSocket)
    {
        string? data = null;
        var buffer = new byte[1024];
        try
        {
            while (clientSocket.Connected)
            {
                var bytesReceived = clientSocket.Receive(buffer);
                data += Encoding.ASCII.GetString(buffer, 0, bytesReceived);
                
                Console.WriteLine($"Received: {data}");

                var response = "+PONG\r\n";
                clientSocket.Send(Encoding.UTF8.GetBytes(response));
            }
            
            clientSocket.Shutdown(SocketShutdown.Both);
            clientSocket.Close();
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.ToString());
        }
    }
}