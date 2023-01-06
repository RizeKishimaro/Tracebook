<?php 
header("Location: index.php");
$media_Files = "Media";
$json_Folder_Name = "Server_Logs";
if(is_dir($json_Folder_Name)):
    $json_root_files = array_filter(scandir($json_Folder_Name),fn($result)=> $result !="." && $result != "..");
    foreach($json_root_files as $json_root_file):
        $read_File_Path = $json_Folder_Name."/".$json_root_file;
        $read_File_Open = fopen($read_File_Path,"r");
        $read_File_Data = fread($read_File_Open,filesize($read_File_Path));
        $read_File_Array = json_decode($read_File_Data,true);
        
        // print_r($read_File_Array);
?>
<div class=" d-flex flex-column-reverse">
<div class=" block-color rounded-3 d-flex flex-column mb-3 shadow">
              <div class=" d-flex align-items-center justify-content-between px-3" style="vertical-align: middle;">
                <div class=" d-flex " style="vertical-align: middle;">
                  <img src="/assests/s-luka.jpg" class=" rounded-circle online-room border border-primary border-3 mx-auto mt-1 ms-2 ms-lg-0" alt="nukasa">
                  <div class=" d-flex flex-column ps-2 custom-fs my-auto">
                    <span class=" text-white fw-bold">Hatsune Miku</span>
                    <span class=" text-white opacity-50 time-fs">4h &diams; <i class=" bi bi-globe"></i></span>
                  </div>
                </div>
                <div class=" d-flex ">
                  <div class=" normal-icon-fs me-3">
                    <i class="bi bi-three-dots"></i>
                  </div>
                  <div class=" normal-icon-fs me-3">
                    <a href="./post-remove.php?id=<?= $read_File_Array['user_id']."&". "file_name=". $read_File_Array['user_files'] ?>" name="ghost"><i class=" bi bi-x-lg"></i></a>
                  </div>
                </div>
              </div>
              <div class=" text-white mt-2 ">
                <p class=" px-3"><?= $read_File_Array["user_Text"]; ?></p>
                <?php 
                $file_Extensions = array("image/png","image/jpeg","image/jpeg","image/webp");
                if(in_array($read_File_Array["file_Type"],$file_Extensions)): ?>
                  <img src="<?= "/".$media_Files."/".$read_File_Array["user_files"]?>" alt="sus" class=" w-100 ">
                <?php else:  ?>
                  <video src="<?= "/".$media_Files."/".$read_File_Array["user_files"]?>" class=" w-100" controls></video>
                <?php endif ?>
              </div>
              <div class="px-3 py-2">
                <div class=" d-flex">
                  <div class="d-flex">
                    <div class=" d-flex flex-row-reverse ms-3 ms-lg-0 me-auto">
                      <div class=" ms-2">
                        <a href="" class=" text-decoration-none page-icon-fade react-counter line-o-hover">Miss Siska and 179 others</a>
                      </div>
                      <p class=" emoji-react text-center rounded-circle emoji-react-leap about-bar-fs-sm">&#129321;</p>
                      <p class=" emoji-react text-center rounded-circle emoji-react-leap about-bar-fs-sm">&#129315;</p>
                      <p class=" emoji-react text-center rounded-circle emoji-react-leap about-bar-fs-sm">&#129304;</p>
                    </div>
                  </div>
                  <div class=" ms-auto">
                    <a href="" class=" page-icon-fade  line-o-hover">18 comment</a>
                    <a href="" class=" page-icon-fade line-o-hover ms-1">27 share</a>
                  </div>
                </div>
                <hr class=" page-icon-fade opacity-50 m-0">

                <div class=" btn-group w-100 d-flex justify-content-center a-control mt-2">
                  <a href="" class=" py-1 h-100 px-3 flex-grow-1 hover-effect text-center rounded-2"><i class=" bi bi-hand-thumbs-up me-2 py-3 "></i> Like</a>
                  <a href="" class=" py-1 h-100 px-3 hover-effect flex-grow-1 text-center rounded-2"><i class=" bi bi-chat-left me-2 py-3"></i> Comment</a>
                  <a href="" class=" py-1 h-100 px-3 hover-effect flex-grow-1 text-center rounded-2"><i class=" bi bi-arrow-90deg-right py-3 me-1"></i> Share</a>
                </div>
              </div>
            </div>
</div>
<?php //include "fileupload.php"; ?>
<?php //include "fileupload.php"; 
fclose($read_File_Open);?>

<?php endforeach; ?>
<?php endif; ?>
            
<?php //$sample = <?= "/".$media_Files."/".$read_File_Array["user_files"] ?>