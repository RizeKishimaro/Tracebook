<?php 
header("Content-type:Application/json");
$dir  ="records";

$fileName = $_GET["file"];

if(!empty($fileName)){

    if(file_exists($dir."/".$fileName)){
        $fileData = json_decode(file_get_contents($dir."/".$fileName),true);
        $fileData["file_name"] = $fileName;
        echo json_encode($fileData);
    }else{
        echo json_encode(["error"=> "file not found"]);
    }

}else{
    echo json_encode(["error" =>"file is required"]);
}

