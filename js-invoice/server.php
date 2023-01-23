<?php 
$server = $_SERVER;
$api_data = json_decode(file_get_contents('php://input'), true);
$file_name = "database.json";
$folder_name = "products";
$fileLocation = $folder_name."/".$file_name;

// $arr = [
//     [
//         "id"=>1,
//         "name"=> "Asashi Umbrella",
//         "price"=> 9500
//     ],
//     [
//         "id"=>2,
//         "name"=> "UwU Mug",
//         "price"=> 4200
//     ],
//     [
//         "id"=>3,
//         "name"=> "Family Guy DVD",
//         "price"=> 4000
//     ],
//     [
//         "id"=>4,
//         "name"=> "Rick And Morty DVD",
//         "price"=> 3600
//     ],
// ];
// print_r($_POST);
// echo "<pre>";
// print_r($server);
if($_SERVER["REQUEST_METHOD"] === "POST"){
    // header("Access-Control-Allow-Origin: *");
    // print_r($api_data);
    header("Content-type: application/json");
    $api_data["name"] = htmlspecialchars($api_data["name"]);

    $post_data = array();
    if(!is_dir($folder_name)){
        mkdir($folder_name);
    }
    if(file_exists($fileLocation)){
        $post_data = json_decode(file_get_contents($fileLocation),true);
    }
    array_push($post_data,$api_data);
    $rawJsonData = json_encode($post_data);
    $stream = fopen($fileLocation,"w");
    fwrite($stream,$rawJsonData);
    fclose($stream);
    http_response_code(200);
    // $jsonData = json_encode($api_data);
    // return $jsonData;
}
if($_SERVER["REQUEST_METHOD"]==="GET"){
   $arr = json_decode(file_get_contents($fileLocation),true);
    // header("Access-Control-Allow-Origin: *");
    header("Content-type: application/json");
    $response = json_encode($arr);
    http_response_code(200);    
    print_r($response);
    return $response;
}
// echo "<pre>";
// print_r($_SERVER);
// error_log(print_r($_POST,true));
?>
