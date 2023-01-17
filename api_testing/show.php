<?php 
header("Content-type:Application/json");

header("Access-Control-Allow-Origin:*");
$dir = "records";

$files = scandir($dir);
// print_r($files);
$result = [];

foreach($files as $file){
    if($file != "." && $file != ".."){

        
        $data = json_decode(file_get_contents($dir."/".$file),true);
        $data["file"] = $file;
        
        array_push($result,$data);
    }
}
$response = json_encode($result);

// echo "<pre>";
echo $response;