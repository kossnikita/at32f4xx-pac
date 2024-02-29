#[doc = "Register `STS2` reader"]
pub type R = crate::R<STS2_SPEC>;
#[doc = "Field `FERRF5` reader - Stream 5 FIFO error interrupt flag"]
pub type FERRF5_R = crate::BitReader;
#[doc = "Field `DMERRF5` reader - Stream 5 direct mode error interrupt flag"]
pub type DMERRF5_R = crate::BitReader;
#[doc = "Field `DTERRF5` reader - Stream 5 transfer error interrupt flag"]
pub type DTERRF5_R = crate::BitReader;
#[doc = "Field `HDTF5` reader - Stream 5 half data transfer interrupt flag"]
pub type HDTF5_R = crate::BitReader;
#[doc = "Field `FDTF5` reader - Stream 5 full data transfer interrupt flag"]
pub type FDTF5_R = crate::BitReader;
#[doc = "Field `FERRF6` reader - Stream 6 FIFO error interrupt flag"]
pub type FERRF6_R = crate::BitReader;
#[doc = "Field `DMERRF6` reader - Stream 6 direct mode error interrupt flag"]
pub type DMERRF6_R = crate::BitReader;
#[doc = "Field `DTERRF6` reader - Stream 6 transfer error interrupt flag"]
pub type DTERRF6_R = crate::BitReader;
#[doc = "Field `HDTF6` reader - Stream 6 half data transfer interrupt flag"]
pub type HDTF6_R = crate::BitReader;
#[doc = "Field `FDTF6` reader - Stream 6 full data transfer interrupt flag"]
pub type FDTF6_R = crate::BitReader;
#[doc = "Field `FERRF7` reader - Stream 7 FIFO error interrupt flag"]
pub type FERRF7_R = crate::BitReader;
#[doc = "Field `DMERRF7` reader - Stream 7 direct mode error interrupt flag"]
pub type DMERRF7_R = crate::BitReader;
#[doc = "Field `DTERRF7` reader - Stream 7 transfer error interrupt flag"]
pub type DTERRF7_R = crate::BitReader;
#[doc = "Field `HDTF7` reader - Stream 7 half data transfer interrupt flag"]
pub type HDTF7_R = crate::BitReader;
#[doc = "Field `FDTF7` reader - Stream 7 full data transfer interrupt flag"]
pub type FDTF7_R = crate::BitReader;
#[doc = "Field `FERRF8` reader - Stream 8 FIFO error interrupt flag"]
pub type FERRF8_R = crate::BitReader;
#[doc = "Field `DMERRF8` reader - Stream 8 direct mode error interrupt flag"]
pub type DMERRF8_R = crate::BitReader;
#[doc = "Field `DTERRF8` reader - Stream 8 transfer error interrupt flag"]
pub type DTERRF8_R = crate::BitReader;
#[doc = "Field `HDTF8` reader - Stream 8 half data transfer interrupt flag"]
pub type HDTF8_R = crate::BitReader;
#[doc = "Field `FDTF8` reader - Stream 8 full data transfer interrupt flag"]
pub type FDTF8_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Stream 5 FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrf5(&self) -> FERRF5_R {
        FERRF5_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Stream 5 direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrf5(&self) -> DMERRF5_R {
        DMERRF5_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stream 5 transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrf5(&self) -> DTERRF5_R {
        DTERRF5_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stream 5 half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtf5(&self) -> HDTF5_R {
        HDTF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stream 5 full data transfer interrupt flag"]
    #[inline(always)]
    pub fn fdtf5(&self) -> FDTF5_R {
        FDTF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stream 6 FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrf6(&self) -> FERRF6_R {
        FERRF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Stream 6 direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrf6(&self) -> DMERRF6_R {
        DMERRF6_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stream 6 transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrf6(&self) -> DTERRF6_R {
        DTERRF6_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Stream 6 half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtf6(&self) -> HDTF6_R {
        HDTF6_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stream 6 full data transfer interrupt flag"]
    #[inline(always)]
    pub fn fdtf6(&self) -> FDTF6_R {
        FDTF6_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Stream 7 FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrf7(&self) -> FERRF7_R {
        FERRF7_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Stream 7 direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrf7(&self) -> DMERRF7_R {
        DMERRF7_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Stream 7 transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrf7(&self) -> DTERRF7_R {
        DTERRF7_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Stream 7 half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtf7(&self) -> HDTF7_R {
        HDTF7_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Stream 7 full data transfer interrupt flag"]
    #[inline(always)]
    pub fn fdtf7(&self) -> FDTF7_R {
        FDTF7_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Stream 8 FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrf8(&self) -> FERRF8_R {
        FERRF8_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Stream 8 direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrf8(&self) -> DMERRF8_R {
        DMERRF8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Stream 8 transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrf8(&self) -> DTERRF8_R {
        DTERRF8_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Stream 8 half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtf8(&self) -> HDTF8_R {
        HDTF8_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Stream 8 full data transfer interrupt flag"]
    #[inline(always)]
    pub fn fdtf8(&self) -> FDTF8_R {
        FDTF8_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS2")
            .field("fdtf8", &format_args!("{}", self.fdtf8().bit()))
            .field("hdtf8", &format_args!("{}", self.hdtf8().bit()))
            .field("dterrf8", &format_args!("{}", self.dterrf8().bit()))
            .field("dmerrf8", &format_args!("{}", self.dmerrf8().bit()))
            .field("ferrf8", &format_args!("{}", self.ferrf8().bit()))
            .field("fdtf7", &format_args!("{}", self.fdtf7().bit()))
            .field("hdtf7", &format_args!("{}", self.hdtf7().bit()))
            .field("dterrf7", &format_args!("{}", self.dterrf7().bit()))
            .field("dmerrf7", &format_args!("{}", self.dmerrf7().bit()))
            .field("ferrf7", &format_args!("{}", self.ferrf7().bit()))
            .field("fdtf6", &format_args!("{}", self.fdtf6().bit()))
            .field("hdtf6", &format_args!("{}", self.hdtf6().bit()))
            .field("dterrf6", &format_args!("{}", self.dterrf6().bit()))
            .field("dmerrf6", &format_args!("{}", self.dmerrf6().bit()))
            .field("ferrf6", &format_args!("{}", self.ferrf6().bit()))
            .field("fdtf5", &format_args!("{}", self.fdtf5().bit()))
            .field("hdtf5", &format_args!("{}", self.hdtf5().bit()))
            .field("dterrf5", &format_args!("{}", self.dterrf5().bit()))
            .field("dmerrf5", &format_args!("{}", self.dmerrf5().bit()))
            .field("ferrf5", &format_args!("{}", self.ferrf5().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<STS2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Interrupt status register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS2_SPEC;
impl crate::RegisterSpec for STS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts2::R`](R) reader structure"]
impl crate::Readable for STS2_SPEC {}
#[doc = "`reset()` method sets STS2 to value 0"]
impl crate::Resettable for STS2_SPEC {
    const RESET_VALUE: u32 = 0;
}
