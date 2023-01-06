<?php 
// header("Content-type: Application/json");
// $file_dir = "Server_Logs";
// if($_SERVER["REQUEST_METHOD"]==="DELETE"){
//     if(!empty($file_dir)){
//     $fileName = $_GET["file_name"];
//     $mime_file_dir = "Media";
//     $mime_file_name = $_GET["mime_file"];
//     if(file_exists($file_dir."/".$fileName) && file_exists($mime_file_dir."/".$mime_file_name)){
//         unlink($file_dir."/".$fileName);
//         unlink($mime_file_dir."/".$mime_file_name);
//         echo json_encode(["success"=>"Requested file Found and deleted"]);
//         header("HTTP/1.1 202 DELETED :'D");
//     }else{
//         header("HTTP/1.1 500 Internal Server Error");
//         echo json_encode(["error"=>"file not found"]);
//     }
// }else{
//     header("Location: 404.html");
// }
// }
if($_SERVER["REQUEST_METHOD"]==="POST"){
   header("Content-type: Application/json");
   $requested_files = $_FILES;
   $folder_name = "Media";
   $serverlogsFiles = "Server_Logs";
   define("MB",20000000);
   if($requested_files["file"]["error"]===0){
      if($requested_files["file"]["size"] >=0 && $requested_files["file"]["size"] <= 5*MB){
         $json_file_name = uniqid().".".pathinfo($requested_files["file"]["name"],PATHINFO_EXTENSION);
         echo json_encode(["file_dir"=>$json_file_name,"file_size"=>$requested_files["file"]["size"]]);
         move_uploaded_file($requested_files["file"]["tmp_file"],$folder_name);
      }else{
         header("HTTP/1.1 413 Payload Too Large");
         echo json_encode(["error"=> "file is too large","file_size"=>$requested_files["file"]["size"]]);
      }
   }
}
// if($_FILES["file"]["error"]===0){
//     $file_name = uniqid().pathinfo($_FILES["photo"]["name"])["extension"];
//     move_uploaded_file($_FILES["file"]["tmp_name"],$folder_name);
//     $result = ["filename"=>$file_name];
// }
?>