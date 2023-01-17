<?php 
if($_SERVER["REQUEST_METHOD"]==="GET"){
    $req_id = $_GET["id"];
    $req_files = $_GET["file_name"];
    print_r($req_files);
    $request_file_name = $req_id.".json";
    $media_Files = "Media";
    $json_Folder_Name = "Server_Logs";
    if(is_file($json_Folder_Name."/".$request_file_name) && is_file($media_Files."/".$req_files)){
        // echo "File deleted";
        unlink($json_Folder_Name."/".$request_file_name);
        unlink($media_Files."/".$req_files);
        header("Location: index.php");
    }else{

        header("Location: 404.html");
    }
    // if(is_dir($json_Folder_Name)){
    //     $json_root_files = array_filter(scandir($json_Folder_Name),fn($result)=> $result !="." && $result != "..");
    //     foreach($json_root_files as $json_root_file){
        
    //     }     
    // } 
}else{
    header("Location: index.php");
}
?>