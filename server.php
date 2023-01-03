<?php include_once "post-model.php" ?>
<?php 
// header("Location: server.php");
$file_Content = $_POST;
$files_Images = $_FILES;
$user_Id = uniqid();
if(isset($file_Content["post_btn"])){
    header("Location: index.php");
    $json_store = json_encode(["user_files"=>$user_Id.".".pathinfo($files_Images["upload_Files"]["name"],PATHINFO_EXTENSION),"file_Type"=>$files_Images["upload_Files"]["type"],"files_Error"=>$files_Images["upload_Files"]["error"],"user_id"=>$user_Id,"user_Text"=>$file_Content["caption_text"]]);
    // print_r($json_store);
    // print_r($file_Content);
    // print_r($files_Images["upload_Files"]["type"]);
    // echo "\n";
    // print_r($files_Images["upload_Files"]['error']);
    
    //storing user data as json
    
    $json_Folder_Name = "Server_Logs";
    $json_File_Name = $user_Id.".json";
    $json_File_Path = $json_Folder_Name."/".$json_File_Name;
    global $json_Folder_Name;
    //creating files
    {
        if(!is_dir($json_Folder_Name)){
            mkdir($json_Folder_Name);
            $server_Logs_Folder = fopen($json_File_Path,"w");
            $server_Logs_Files = fwrite($server_Logs_Folder,$json_store);
            fclose($server_Logs_Folder);
            // fclose($server_Logs_Folder);
        }else{
            $server_Logs_Folder = fopen($json_File_Path,"w");
            $server_Logs_Files = fwrite($server_Logs_Folder,$json_store);
            fclose($server_Logs_Folder);
        }
        
        if($files_Images["upload_Files"]["error"] === 0){
            if($files_Images["upload_Files"]["size"] > 0){
                $file_Extensions = array("image/png","image/jpeg","image/jpeg","image/webp","video/mp4","video/mkv");
                if(in_array($files_Images["upload_Files"]["type"],$file_Extensions)){
                    //create folder for storing files
                    $media_Folder = "Media/";
                    //check if the file is exist
                    if(!is_dir($media_Folder)){
                        mkdir($media_Folder);
                    }
                    //making user file name
                    $users_Files_Path = $user_Id.".".pathinfo($files_Images["upload_Files"]["name"],PATHINFO_EXTENSION);//taking file extension
                    //making file upload to target file
                    $store_File_Path = move_uploaded_file($files_Images["upload_Files"]["tmp_name"],$media_Folder.$users_Files_Path);
                    if($store_File_Path){
                        // echo "uploaded successfully";
                        // echo var_dump($store_File_Path);
                    }else{
                        // $err_Var = "Your Files upload failed!!!!";
                        // echo $err_Var;
                    }

                }else{
                    // echo "You should upload image video files not other files!";
                }
            }
            else{
                // echo "This files are invalid";
            }
        }else{
            // echo "Your files was invalid the server removed it!";
        }
    }
}
?>
