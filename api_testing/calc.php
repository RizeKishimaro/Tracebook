<?php 
header("Content-type:Application/json");

$dir = "records";
$data = $_POST;

if(!is_dir($dir)){
    mkdir($dir);
}

if($_SERVER["REQUEST_METHOD"]=== "POST"){
    $results = [
        "width" => $data["width"]." ft",
        "bredth" => $data["breadth"]." ft",
        "area" => $data["width"]*$data["breadth"]." sqft"
    ];
    $data = json_encode($results);
}

$newFileLocationName =$dir."/".md5(uniqid()).".json";

$steam = fopen($newFileLocationName,"w");
fwrite($steam,$data);
fclose($steam);

echo $data;

