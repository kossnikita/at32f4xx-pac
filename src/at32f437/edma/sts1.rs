#[doc = "Register `STS1` reader"]
pub type R = crate::R<STS1_SPEC>;
#[doc = "Field `FERRF1` reader - Stream 1 FIFO error interrupt flag"]
pub type FERRF1_R = crate::BitReader;
#[doc = "Field `DMERRF1` reader - Stream 1 direct mode error interrupt flag"]
pub type DMERRF1_R = crate::BitReader;
#[doc = "Field `DTERRF1` reader - Stream 1 transfer error interrupt flag"]
pub type DTERRF1_R = crate::BitReader;
#[doc = "Field `HDTF1` reader - Stream 1 half data transfer interrupt flag"]
pub type HDTF1_R = crate::BitReader;
#[doc = "Field `FDTF1` reader - Stream 1 Full data transfer interrupt flag"]
pub type FDTF1_R = crate::BitReader;
#[doc = "Field `FERRF2` reader - Stream 2 FIFO error interrupt flag"]
pub type FERRF2_R = crate::BitReader;
#[doc = "Field `DMERRF2` reader - Stream 2 direct mode error interrupt flag"]
pub type DMERRF2_R = crate::BitReader;
#[doc = "Field `DTERRF2` reader - Stream 2 transfer error interrupt flag"]
pub type DTERRF2_R = crate::BitReader;
#[doc = "Field `HDTF2` reader - Stream 2 half data transfer interrupt flag"]
pub type HDTF2_R = crate::BitReader;
#[doc = "Field `FDTF2` reader - Stream 2 Full data transfer interrupt flag"]
pub type FDTF2_R = crate::BitReader;
#[doc = "Field `FERRF3` reader - Stream 3 FIFO error interrupt flag"]
pub type FERRF3_R = crate::BitReader;
#[doc = "Field `DMERRF3` reader - Stream 3 direct mode error interrupt flag"]
pub type DMERRF3_R = crate::BitReader;
#[doc = "Field `DTERRF3` reader - Stream 3 transfer error interrupt flag"]
pub type DTERRF3_R = crate::BitReader;
#[doc = "Field `HDTF3` reader - Stream 3 half data transfer interrupt flag"]
pub type HDTF3_R = crate::BitReader;
#[doc = "Field `FDTF3` reader - Stream 3 Full data transfer interrupt flag"]
pub type FDTF3_R = crate::BitReader;
#[doc = "Field `FERRF4` reader - Stream 4 FIFO error interrupt flag"]
pub type FERRF4_R = crate::BitReader;
#[doc = "Field `DMERRF4` reader - Stream 4 direct mode error interrupt flag"]
pub type DMERRF4_R = crate::BitReader;
#[doc = "Field `DTERRF4` reader - Stream 4 transfer error interrupt flag"]
pub type DTERRF4_R = crate::BitReader;
#[doc = "Field `HDTF4` reader - Stream 4 half data transfer interrupt flag"]
pub type HDTF4_R = crate::BitReader;
#[doc = "Field `FDTF4` reader - Stream 4 Full data transfer interrupt flag"]
pub type FDTF4_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Stream 1 FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrf1(&self) -> FERRF1_R {
        FERRF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Stream 1 direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrf1(&self) -> DMERRF1_R {
        DMERRF1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stream 1 transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrf1(&self) -> DTERRF1_R {
        DTERRF1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stream 1 half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtf1(&self) -> HDTF1_R {
        HDTF1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stream 1 Full data transfer interrupt flag"]
    #[inline(always)]
    pub fn fdtf1(&self) -> FDTF1_R {
        FDTF1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stream 2 FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrf2(&self) -> FERRF2_R {
        FERRF2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Stream 2 direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrf2(&self) -> DMERRF2_R {
        DMERRF2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stream 2 transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrf2(&self) -> DTERRF2_R {
        DTERRF2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Stream 2 half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtf2(&self) -> HDTF2_R {
        HDTF2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stream 2 Full data transfer interrupt flag"]
    #[inline(always)]
    pub fn fdtf2(&self) -> FDTF2_R {
        FDTF2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Stream 3 FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrf3(&self) -> FERRF3_R {
        FERRF3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Stream 3 direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrf3(&self) -> DMERRF3_R {
        DMERRF3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Stream 3 transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrf3(&self) -> DTERRF3_R {
        DTERRF3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Stream 3 half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtf3(&self) -> HDTF3_R {
        HDTF3_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Stream 3 Full data transfer interrupt flag"]
    #[inline(always)]
    pub fn fdtf3(&self) -> FDTF3_R {
        FDTF3_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Stream 4 FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrf4(&self) -> FERRF4_R {
        FERRF4_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Stream 4 direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrf4(&self) -> DMERRF4_R {
        DMERRF4_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Stream 4 transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrf4(&self) -> DTERRF4_R {
        DTERRF4_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Stream 4 half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtf4(&self) -> HDTF4_R {
        HDTF4_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Stream 4 Full data transfer interrupt flag"]
    #[inline(always)]
    pub fn fdtf4(&self) -> FDTF4_R {
        FDTF4_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS1")
            .field("fdtf4", &format_args!("{}", self.fdtf4().bit()))
            .field("hdtf4", &format_args!("{}", self.hdtf4().bit()))
            .field("dterrf4", &format_args!("{}", self.dterrf4().bit()))
            .field("dmerrf4", &format_args!("{}", self.dmerrf4().bit()))
            .field("ferrf4", &format_args!("{}", self.ferrf4().bit()))
            .field("fdtf3", &format_args!("{}", self.fdtf3().bit()))
            .field("hdtf3", &format_args!("{}", self.hdtf3().bit()))
            .field("dterrf3", &format_args!("{}", self.dterrf3().bit()))
            .field("dmerrf3", &format_args!("{}", self.dmerrf3().bit()))
            .field("ferrf3", &format_args!("{}", self.ferrf3().bit()))
            .field("fdtf2", &format_args!("{}", self.fdtf2().bit()))
            .field("hdtf2", &format_args!("{}", self.hdtf2().bit()))
            .field("dterrf2", &format_args!("{}", self.dterrf2().bit()))
            .field("dmerrf2", &format_args!("{}", self.dmerrf2().bit()))
            .field("ferrf2", &format_args!("{}", self.ferrf2().bit()))
            .field("fdtf1", &format_args!("{}", self.fdtf1().bit()))
            .field("hdtf1", &format_args!("{}", self.hdtf1().bit()))
            .field("dterrf1", &format_args!("{}", self.dterrf1().bit()))
            .field("dmerrf1", &format_args!("{}", self.dmerrf1().bit()))
            .field("ferrf1", &format_args!("{}", self.ferrf1().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<STS1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Interrupt status register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS1_SPEC;
impl crate::RegisterSpec for STS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts1::R`](R) reader structure"]
impl crate::Readable for STS1_SPEC {}
#[doc = "`reset()` method sets STS1 to value 0"]
impl crate::Resettable for STS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
