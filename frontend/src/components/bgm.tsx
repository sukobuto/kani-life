import styled from "styled-components";
import {useState} from "react";

function Bgm() {
    const [opened, setOpened] = useState(false);
    const [videoId, setVideoId] = useState<string | undefined>();
    return (
        <BgmFrame $opened={opened}>
            <BgmButton onClick={() => setOpened(!opened)}>🎶</BgmButton>
            {videoId && (
                <iframe width="560" height="315"
                        src={`https://www.youtube.com/embed/${videoId}?mute=1&loop=1&playlist=${videoId}`}
                        title="YouTube video player" frameBorder="0"
                        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                        referrerPolicy="strict-origin-when-cross-origin"></iframe>
            )}
            {opened && (
                <VideoIdInput type="text" placeholder="YouTube Video ID" value={videoId}
                              onChange={(e) => setVideoId(e.target.value)}/>
            )}
        </BgmFrame>
    );
}

const BgmFrame = styled.div<{ $opened: boolean; }>`
    position: fixed;
    bottom: 0;
    right: 0;

    iframe {
        visibility: ${(props) => props.$opened ? 'visible' : 'hidden'};
        width: ${(props) => props.$opened ? '560px' : '0'};
        margin-bottom: 30px;
    }
`

const BgmButton = styled.button`
    position: fixed;
    bottom: 0;
    right: 0;
    height: 35px;
    line-height: 10px;
`

const VideoIdInput = styled.input`
    position: fixed;
    bottom: 0;
    right: 60px;
    width: 200px;
    height: 30px;
`

export default Bgm;
