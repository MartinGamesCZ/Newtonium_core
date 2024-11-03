// @bun
import{dlopen as G,FFIType as j}from"bun:ffi";import{randomUUID as J}from"crypto";self.onmessage=(q)=>{const k=G(q.data.lib_path,{initialize:{args:[],returns:j.bool},run:{args:[],returns:j.void},create_window:{args:[j.cstring,j.cstring,j.cstring],returns:j.ptr},create_element:{args:[j.ptr,j.cstring,j.cstring,j.cstring],returns:j.void},append_child:{args:[j.ptr,j.cstring,j.ptr],returns:j.void}});if(q.data.is_new){if(!k.symbols.initialize())console.error("Failed to initialize library"),self.terminate();console.log("Initialized library");let v=new Uint8Array([...new TextEncoder().encode("/home/marti/icon.png"),0]),B=new Uint8Array([...new TextEncoder().encode("win"),0]),C=new Uint8Array([...new TextEncoder().encode("mega window"),0]);const z=k.symbols.create_window(C,v,B);console.log(z),postMessage({e:"new",ptr:z}),k.symbols.run()}else{const v=J().replaceAll("-","");k.symbols.create_element(q.data.ptr,new Uint8Array([...new TextEncoder().encode("text"),0]),new Uint8Array([...new TextEncoder().encode(v),0]),new Uint8Array([...new TextEncoder().encode("Hello, World!"),0])),k.symbols.append_child(q.data.ptr,new Uint8Array([...new TextEncoder().encode("body"),0]),new Uint8Array([...new TextEncoder().encode(v),0]))}};

//# debugId=9C2FAE528297983964756E2164756E21
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsiLi4vc3JjL3dvcmtlci50cyJdLAogICJzb3VyY2VzQ29udGVudCI6IFsKICAgICJpbXBvcnQgeyBDRnVuY3Rpb24sIGRsb3BlbiwgRkZJVHlwZSB9IGZyb20gXCJidW46ZmZpXCI7XG5pbXBvcnQgeyByYW5kb21VVUlEIH0gZnJvbSBcImNyeXB0b1wiO1xuXG5kZWNsYXJlIHZhciBzZWxmOiBXb3JrZXI7XG5cbnNlbGYub25tZXNzYWdlID0gKGUpID0+IHtcbiAgY29uc3QgbGliID0gZGxvcGVuKGUuZGF0YS5saWJfcGF0aCwge1xuICAgIGluaXRpYWxpemU6IHtcbiAgICAgIGFyZ3M6IFtdLFxuICAgICAgcmV0dXJuczogRkZJVHlwZS5ib29sLFxuICAgIH0sXG4gICAgcnVuOiB7XG4gICAgICBhcmdzOiBbXSxcbiAgICAgIHJldHVybnM6IEZGSVR5cGUudm9pZCxcbiAgICB9LFxuICAgIGNyZWF0ZV93aW5kb3c6IHtcbiAgICAgIGFyZ3M6IFtGRklUeXBlLmNzdHJpbmcsIEZGSVR5cGUuY3N0cmluZywgRkZJVHlwZS5jc3RyaW5nXSxcbiAgICAgIHJldHVybnM6IEZGSVR5cGUucHRyLFxuICAgIH0sXG4gICAgY3JlYXRlX2VsZW1lbnQ6IHtcbiAgICAgIGFyZ3M6IFtGRklUeXBlLnB0ciwgRkZJVHlwZS5jc3RyaW5nLCBGRklUeXBlLmNzdHJpbmcsIEZGSVR5cGUuY3N0cmluZ10sXG4gICAgICByZXR1cm5zOiBGRklUeXBlLnZvaWQsXG4gICAgfSxcbiAgICBhcHBlbmRfY2hpbGQ6IHtcbiAgICAgIGFyZ3M6IFtGRklUeXBlLnB0ciwgRkZJVHlwZS5jc3RyaW5nLCBGRklUeXBlLnB0cl0sXG4gICAgICByZXR1cm5zOiBGRklUeXBlLnZvaWQsXG4gICAgfSxcbiAgfSk7XG5cbiAgaWYgKGUuZGF0YS5pc19uZXcpIHtcbiAgICBpZiAoIWxpYi5zeW1ib2xzLmluaXRpYWxpemUoKSkge1xuICAgICAgY29uc29sZS5lcnJvcihcIkZhaWxlZCB0byBpbml0aWFsaXplIGxpYnJhcnlcIik7XG4gICAgICBzZWxmLnRlcm1pbmF0ZSgpO1xuICAgIH1cblxuICAgIGNvbnNvbGUubG9nKFwiSW5pdGlhbGl6ZWQgbGlicmFyeVwiKTtcblxuICAgIGxldCBpY29uID0gbmV3IFVpbnQ4QXJyYXkoW1xuICAgICAgLi4ubmV3IFRleHRFbmNvZGVyKCkuZW5jb2RlKFwiL2hvbWUvbWFydGkvaWNvbi5wbmdcIiksXG4gICAgICAwLFxuICAgIF0pO1xuXG4gICAgbGV0IGlkID0gbmV3IFVpbnQ4QXJyYXkoWy4uLm5ldyBUZXh0RW5jb2RlcigpLmVuY29kZShcIndpblwiKSwgMF0pO1xuICAgIGxldCB0aXRsZSA9IG5ldyBVaW50OEFycmF5KFsuLi5uZXcgVGV4dEVuY29kZXIoKS5lbmNvZGUoXCJtZWdhIHdpbmRvd1wiKSwgMF0pO1xuICAgIGNvbnN0IHB0ciA9IGxpYi5zeW1ib2xzLmNyZWF0ZV93aW5kb3codGl0bGUsIGljb24sIGlkKTtcblxuICAgIGNvbnNvbGUubG9nKHB0cik7XG5cbiAgICBwb3N0TWVzc2FnZSh7XG4gICAgICBlOiBcIm5ld1wiLFxuICAgICAgcHRyLFxuICAgIH0pO1xuXG4gICAgbGliLnN5bWJvbHMucnVuKCk7XG4gIH0gZWxzZSB7XG4gICAgY29uc3QgaWQgPSByYW5kb21VVUlEKCkucmVwbGFjZUFsbChcIi1cIiwgXCJcIik7XG5cbiAgICBsaWIuc3ltYm9scy5jcmVhdGVfZWxlbWVudChcbiAgICAgIGUuZGF0YS5wdHIsXG4gICAgICBuZXcgVWludDhBcnJheShbLi4ubmV3IFRleHRFbmNvZGVyKCkuZW5jb2RlKFwidGV4dFwiKSwgMF0pLFxuICAgICAgbmV3IFVpbnQ4QXJyYXkoWy4uLm5ldyBUZXh0RW5jb2RlcigpLmVuY29kZShpZCksIDBdKSxcbiAgICAgIG5ldyBVaW50OEFycmF5KFsuLi5uZXcgVGV4dEVuY29kZXIoKS5lbmNvZGUoXCJIZWxsbywgV29ybGQhXCIpLCAwXSlcbiAgICApO1xuXG4gICAgbGliLnN5bWJvbHMuYXBwZW5kX2NoaWxkKFxuICAgICAgZS5kYXRhLnB0cixcbiAgICAgIG5ldyBVaW50OEFycmF5KFsuLi5uZXcgVGV4dEVuY29kZXIoKS5lbmNvZGUoXCJib2R5XCIpLCAwXSksXG4gICAgICBuZXcgVWludDhBcnJheShbLi4ubmV3IFRleHRFbmNvZGVyKCkuZW5jb2RlKGlkKSwgMF0pXG4gICAgKTtcbiAgfVxuXG4gIC8qbGV0IGlkID0gbmV3IFRleHRFbmNvZGVyKCkuZW5jb2RlKGUuZGF0YS5pZCk7XG5cbiAgaWQgPSBuZXcgVWludDhBcnJheShbLi4uaWQsIDBdKTtcblxuICBsZXQgYXBwID0gbnVsbDtcblxuICB0cnkge1xuICAgIGFwcCA9IGxpYi5zeW1ib2xzLmNyZWF0ZV9hcHBsaWNhdGlvbihpZCk7XG4gIH0gY2F0Y2ggKGUpIHtcbiAgICBjb25zb2xlLmxvZyhlKTtcbiAgfVxuXG4gIGlmICghYXBwKSB0aHJvdyBuZXcgRXJyb3IoXCJObyB3aW5kb3dcIik7XG5cbiAgdHJ5IHtcbiAgICBsZXQgdGl0bGUgPSBuZXcgVGV4dEVuY29kZXIoKS5lbmNvZGUoXCJIZWxsbywgV29ybGQhXCIpO1xuICAgIHRpdGxlID0gbmV3IFVpbnQ4QXJyYXkoWy4uLnRpdGxlLCAwXSk7XG5cbiAgICBsZXQgbyA9IGxpYi5zeW1ib2xzLmNyZWF0ZV93aW5kb3coYXBwLCB0aXRsZSk7XG5cbiAgICBjb25zb2xlLmxvZyhvKTtcbiAgfSBjYXRjaCAoZSkge1xuICAgIGNvbnNvbGUubG9nKGUpO1xuICB9XG5cbiAgLypwb3N0TWVzc2FnZSh7XG4gICAgZTogXCJjbG9zZVwiLFxuICB9KTsqL1xufTtcbiIKICBdLAogICJtYXBwaW5ncyI6ICI7QUFBQSxpQkFBb0IsYUFBUSxnQkFDNUIscUJBQVMsZUFJVCxLQUFLLFVBQVksQ0FBQyxJQUFNLENBQ3RCLE1BQU0sRUFBTSxFQUFPLEVBQUUsS0FBSyxTQUFVLENBQ2xDLFdBQVksQ0FDVixLQUFNLENBQUMsRUFDUCxRQUFTLEVBQVEsSUFDbkIsRUFDQSxJQUFLLENBQ0gsS0FBTSxDQUFDLEVBQ1AsUUFBUyxFQUFRLElBQ25CLEVBQ0EsY0FBZSxDQUNiLEtBQU0sQ0FBQyxFQUFRLFFBQVMsRUFBUSxRQUFTLEVBQVEsT0FBTyxFQUN4RCxRQUFTLEVBQVEsR0FDbkIsRUFDQSxlQUFnQixDQUNkLEtBQU0sQ0FBQyxFQUFRLElBQUssRUFBUSxRQUFTLEVBQVEsUUFBUyxFQUFRLE9BQU8sRUFDckUsUUFBUyxFQUFRLElBQ25CLEVBQ0EsYUFBYyxDQUNaLEtBQU0sQ0FBQyxFQUFRLElBQUssRUFBUSxRQUFTLEVBQVEsR0FBRyxFQUNoRCxRQUFTLEVBQVEsSUFDbkIsQ0FDRixDQUFDLEVBRUQsR0FBSSxFQUFFLEtBQUssT0FBUSxDQUNqQixJQUFLLEVBQUksUUFBUSxXQUFXLEVBQzFCLFFBQVEsTUFBTSw4QkFBOEIsRUFDNUMsS0FBSyxVQUFVLEVBR2pCLFFBQVEsSUFBSSxxQkFBcUIsRUFFakMsSUFBSSxFQUFPLElBQUksV0FBVyxDQUN4QixHQUFHLElBQUksWUFBWSxFQUFFLE9BQU8sc0JBQXNCLEVBQ2xELENBQ0YsQ0FBQyxFQUVHLEVBQUssSUFBSSxXQUFXLENBQUMsR0FBRyxJQUFJLFlBQVksRUFBRSxPQUFPLEtBQUssRUFBRyxDQUFDLENBQUMsRUFDM0QsRUFBUSxJQUFJLFdBQVcsQ0FBQyxHQUFHLElBQUksWUFBWSxFQUFFLE9BQU8sYUFBYSxFQUFHLENBQUMsQ0FBQyxFQUMxRSxNQUFNLEVBQU0sRUFBSSxRQUFRLGNBQWMsRUFBTyxFQUFNLENBQUUsRUFFckQsUUFBUSxJQUFJLENBQUcsRUFFZixZQUFZLENBQ1YsRUFBRyxNQUNILEtBQ0YsQ0FBQyxFQUVELEVBQUksUUFBUSxJQUFJLE1BQ1gsQ0FDTCxNQUFNLEVBQUssRUFBVyxFQUFFLFdBQVcsSUFBSyxFQUFFLEVBRTFDLEVBQUksUUFBUSxlQUNWLEVBQUUsS0FBSyxJQUNQLElBQUksV0FBVyxDQUFDLEdBQUcsSUFBSSxZQUFZLEVBQUUsT0FBTyxNQUFNLEVBQUcsQ0FBQyxDQUFDLEVBQ3ZELElBQUksV0FBVyxDQUFDLEdBQUcsSUFBSSxZQUFZLEVBQUUsT0FBTyxDQUFFLEVBQUcsQ0FBQyxDQUFDLEVBQ25ELElBQUksV0FBVyxDQUFDLEdBQUcsSUFBSSxZQUFZLEVBQUUsT0FBTyxlQUFlLEVBQUcsQ0FBQyxDQUFDLENBQ2xFLEVBRUEsRUFBSSxRQUFRLGFBQ1YsRUFBRSxLQUFLLElBQ1AsSUFBSSxXQUFXLENBQUMsR0FBRyxJQUFJLFlBQVksRUFBRSxPQUFPLE1BQU0sRUFBRyxDQUFDLENBQUMsRUFDdkQsSUFBSSxXQUFXLENBQUMsR0FBRyxJQUFJLFlBQVksRUFBRSxPQUFPLENBQUUsRUFBRyxDQUFDLENBQUMsQ0FDckQiLAogICJkZWJ1Z0lkIjogIjlDMkZBRTUyODI5Nzk4Mzk2NDc1NkUyMTY0NzU2RTIxIiwKICAibmFtZXMiOiBbXQp9