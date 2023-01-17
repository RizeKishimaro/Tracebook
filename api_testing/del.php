<?php 
header("Content-type:Application/json");
$dir  ="records";

$fileName = $_GET["file"];

if($_SERVER["REQUEST_METHOD"]==="DELETE"){
    if(!empty($fileName)){

        if(file_exists($dir."/".$fileName)){
            $fileData = json_decode(file_get_contents($dir."/".$fileName),true);
            $fileData["file_name"] = $fileName;
            json_encode($fileData);
            unlink($dir."/".$fileName);
        }else{
            echo json_encode(["error"=> "file not found"]);
        }
    
    }else{
        echo json_encode(["error" =>"file is required"]);
    }
}

