#pragma once
#ifndef CRYFS_LIB_IMPL_SYMLINKBLOB_H_
#define CRYFS_LIB_IMPL_SYMLINKBLOB_H_

#include <messmer/blobstore/interface/Blob.h>
#include <boost/filesystem/path.hpp>
#include <messmer/cpp-utils/pointer/unique_ref.h>

namespace cryfs {

class SymlinkBlob {
public:
  static cpputils::unique_ref<SymlinkBlob> InitializeSymlink(cpputils::unique_ref<blobstore::Blob> blob, const boost::filesystem::path &target);

  SymlinkBlob(cpputils::unique_ref<blobstore::Blob> blob);
  SymlinkBlob(const boost::filesystem::path &target);
  virtual ~SymlinkBlob();

  const boost::filesystem::path &target() const;

private:
  boost::filesystem::path _target;

  static void _checkMagicNumber(const blobstore::Blob &blob);
  static boost::filesystem::path _readTargetFromBlob(const blobstore::Blob &blob);
};

}

#endif
