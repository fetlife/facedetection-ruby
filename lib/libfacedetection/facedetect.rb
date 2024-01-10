module Libfacedetection
  module FaceDetection
    require 'ruby-vips'
    require 'ffi'

    LIB_FACEDETECTION_MAX_SIZE = 400.0

    extend self

    # vips_img: Vips::Image
    def detect_faces(vips_img)
      scale =
      if vips_img.width > vips_img.height
        LIB_FACEDETECTION_MAX_SIZE / vips_img.width
      else
        LIB_FACEDETECTION_MAX_SIZE / vips_img.height
      end
      # convert RGB -> BGR
      rgb_bands = vips_img.bandsplit
      bgr = Vips::Image.bandjoin([rgb_bands[2], rgb_bands[1], rgb_bands[0]])
      # resize to a smaller size - libfacedetection works with smaller images
      # or perhaps a larger one, but results are not guaranteed to be better
      resized = bgr.resize(scale)

      mem = resized.write_to_memory
      ptr = FFI::MemoryPointer.from_string(mem)
      faces = Libfacedetection.detect_libfacedetection_image_data(ptr.address, resized.width, resized.height)
      faces.map do |face|
        scale_face(face, scale)
      end
    end

    private

    def scale_face(face, scale)
      face[:x] = (face[:x] / scale).round
      face[:y] = (face[:y] / scale).round
      face[:width] = (face[:width] / scale).round
      face[:height] = (face[:height] / scale).round
      face[:landmarks] = face[:landmarks].map do |landmark|
        [(landmark[0] / scale).round, (landmark[1] / scale).round]
      end
      face
    end
  end

end
