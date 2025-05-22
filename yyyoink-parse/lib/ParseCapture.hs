{-# LANGUAGE TypeApplications #-}

module ParseCapture where

import qualified Control.Exception as Exception
import qualified Data.Text as Text
import qualified Data.Text.IO as TextIO
import qualified System.Environment as Env
import qualified System.IO.Error as IOError

runParseCapture :: IO ()
runParseCapture =
  handleIOError $
    handleArgs
      >>= eitherToErr
      >>= TextIO.readFile
      >>= TextIO.putStrLn
  where
    handleIOError :: IO () -> IO ()
    handleIOError ioAction =
      Exception.catch ioAction $
        \e -> putStrLn "An error occurred: " >> print @IOError e

handleArgs :: IO (Either String FilePath)
handleArgs =
  parseArgs <$> Env.getArgs
  where
    parseArgs argumentList =
      case argumentList of
        [fname] -> Right fname
        [] -> Left "Failed: Must input filename"
        _ -> Left "Failed: Cannot input multiple files"

eitherToErr :: (Show a) => Either a b -> IO b
eitherToErr (Right a) = return a
eitherToErr (Left e) =
  Exception.throwIO . IOError.userError $ show e
